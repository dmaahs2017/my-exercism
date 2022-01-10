use std::collections::HashMap;
/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<T> = Box<dyn Fn(&[T]) -> T>;

pub struct Reactor<T> {
    id_generator: IdGenerator,
    cells: HashMap<CellId, T>,
    notify_on_update: HashMap<CellId, Vec<CellId>>,
    compute_cell_funcs: HashMap<CellId, (Vec<CellId>, ComputeFunc<T>)>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<T: Copy + PartialEq> Reactor<T> {
    pub fn new() -> Self {
        Self {
            id_generator: IdGenerator::new(),
            cells: Default::default(),
            notify_on_update: Default::default(),
            compute_cell_funcs: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = self.id_generator.input_id();
        self.cells.insert(CellId::Input(id), initial);
        id
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
    pub fn create_compute<F: Fn(&[T]) -> T + 'static>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = self.id_generator.compute_id();
        let func = Box::new(compute_func);
        let values = dependencies
            .iter()
            .map(|&id| self.value(id).ok_or(id))
            .collect::<Result<Vec<T>, CellId>>()?;
        let init_value = func(&values);
        self.cells.insert(CellId::Compute(id), init_value);

        for &dep in dependencies {
            self.notify_on_update
                .entry(dep)
                .or_default()
                .push(CellId::Compute(id));
        }
        self.compute_cell_funcs
            .insert(CellId::Compute(id), (dependencies.to_vec(), func));
        Ok(id)
    }

    fn recalculate_compute_cell(&mut self, compute_id: CellId) {
        let (inputs, func) = self.compute_cell_funcs.get(&compute_id).unwrap();

        let values = inputs
            .iter()
            .map(|&id| self.value(id).unwrap())
            .collect::<Vec<T>>();
        self.cells.get_mut(&compute_id).map(|v| *v = func(&values));

        if let Some(ids) = self.notify_on_update.get(&compute_id).cloned() {
            for id in ids {
                self.recalculate_compute_cell(id)
            }
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        self.cells.get(&id).cloned()
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
        let id = CellId::Input(id);
        let success = self.cells.get_mut(&id).map(|v| *v = new_value).is_some();

        if let Some(compute_ids) = self.notify_on_update.get(&id).cloned() {
            for compute_id in compute_ids {
                self.recalculate_compute_cell(compute_id);
            }
        }

        success
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
    pub fn add_callback<F: FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        unimplemented!()
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
        unimplemented!(
            "Remove the callback identified by the CallbackId {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}

struct IdGenerator {
    next: usize,
}

impl IdGenerator {
    fn new() -> Self {
        Self { next: 0 }
    }

    fn next(&mut self) -> usize {
        let v = self.next;
        self.next += 1;
        v
    }

    fn input_id(&mut self) -> InputCellId {
        InputCellId(self.next())
    }

    fn compute_id(&mut self) -> ComputeCellId {
        ComputeCellId(self.next())
    }

    fn callback_id(&mut self) -> CallbackId {
        CallbackId(self.next())
    }
}
