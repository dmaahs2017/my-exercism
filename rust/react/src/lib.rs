use std::cell::{Cell, RefCell};
use std::collections::HashSet;
/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct InputCellId(usize);
/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<'a, T> = Box<dyn Fn(&[T]) -> T + 'a>;
type CallbackFunc<'a, T> = Box<RefCell<dyn FnMut(T) + 'a>>;

struct ComputeCell<'a, T> {
    value: Cell<T>,
    function: ComputeFunc<'a, T>,
    dependencies: Vec<CellId>,
    callbacks: Vec<Option<CallbackFunc<'a, T>>>,
}

impl<'a, T> ComputeCell<'a, T> {
    fn new<F: Fn(&[T]) -> T + 'a>(value: T, function: F, dependencies: &[CellId]) -> Self {
        Self {
            value: Cell::new(value),
            function: Box::new(function),
            dependencies: dependencies.to_vec(),
            callbacks: Default::default(),
        }
    }
}

pub struct Reactor<'a, T> {
    input: Vec<T>,
    compute: Vec<ComputeCell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            input: Default::default(),
            compute: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        self.input.push(initial);
        InputCellId(self.input.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: Fn(&[T]) -> T + 'a>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let value = compute_func(&self.get_dependency_values(dependencies)?);
        let compute_cell = ComputeCell::new(value, compute_func, dependencies);
        self.compute.push(compute_cell);
        Ok(ComputeCellId(self.compute.len() - 1))
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.input.get(id.0).cloned(),
            CellId::Compute(id) => self
                .compute
                .get(id.0)
                .map(|compute_cell| compute_cell.value.get()),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        match self.input.get_mut(id.0) {
            Some(cell) => *cell = new_value,
            None => return false,
        }

        let mut changed = HashSet::new();
        changed.insert(CellId::Input(id));
        for (i, compute_cell) in self.compute.iter().enumerate() {
            if compute_cell
                .dependencies
                .iter()
                .any(|c| changed.contains(c))
            {
                let dep_values = self
                    .get_dependency_values(&compute_cell.dependencies)
                    .unwrap();
                let new_value = (compute_cell.function)(&dep_values);
                if new_value != compute_cell.value.get() {
                    compute_cell.value.set(new_value);
                    changed.insert(CellId::Compute(ComputeCellId(i)));
                    for callback in &compute_cell.callbacks {
                        if let Some(callback) = callback {
                            callback.borrow_mut()(new_value);
                        }
                    }
                }
            }
        }

        true
    }

    fn get_dependency_values(&self, deps: &[CellId]) -> Result<Vec<T>, CellId> {
        deps.iter().map(|&id| self.value(id).ok_or(id)).collect()
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(compute) = self.compute.get_mut(id.0) {
            compute
                .callbacks
                .push(Some(Box::new(RefCell::new(callback))));
            return Some(CallbackId(compute.callbacks.len() - 1));
        }
        None
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(cell) = self.compute.get_mut(cell.0) {
            if let Some(cb) = cell.callbacks.get_mut(callback.0) {
                if cb.is_none() {
                    return Err(RemoveCallbackError::NonexistentCallback);
                } else {
                    *cb = None;
                }
            } else {
                return Err(RemoveCallbackError::NonexistentCallback);
            }
        } else {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        Ok(())
    }
}
