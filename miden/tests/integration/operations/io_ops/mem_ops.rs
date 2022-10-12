use super::{build_op_test, build_test};

// PUSHING VALUES ONTO THE STACK (PUSH)
// ================================================================================================

#[test]
fn mem_load() {
    let addr = 1;
    let asm_op = "mem_load";

    // --- read from uninitialized memory - address provided via the stack ------------------------
    let test = build_op_test!(asm_op, &[addr]);
    test.expect_stack(&[0]);

    // --- read from uninitialized memory - address provided as a parameter -----------------------
    let asm_op = format!("{}.{}", asm_op, addr);
    let test = build_op_test!(&asm_op);
    test.expect_stack(&[0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    let test = build_op_test!(&asm_op, &[1, 2, 3, 4]);
    test.expect_stack(&[0, 4, 3, 2, 1]);
}

// REMOVING VALUES FROM THE STACK (POP)
// ================================================================================================

#[test]
fn mem_store() {
    let asm_op = "mem_store";
    let addr = 0;

    // --- address provided via the stack ---------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 2, 3, 4, addr]);
    test.expect_stack_and_memory(&[4, 3, 2, 1], addr, &[4, 0, 0, 0]);

    // --- address provided as a parameter --------------------------------------------------------
    let asm_op = format!("{}.{}", asm_op, addr);
    let test = build_op_test!(&asm_op, &[1, 2, 3, 4]);
    test.expect_stack_and_memory(&[4, 3, 2, 1], addr, &[4, 0, 0, 0]);
}

// OVERWRITING VALUES ON THE STACK (LOAD)
// ================================================================================================

#[test]
fn mem_loadw() {
    let addr = 1;
    let asm_op = "mem_loadw";

    // --- read from uninitialized memory - address provided via the stack ------------------------
    let test = build_op_test!(asm_op, &[addr, 5, 6, 7, 8]);
    test.expect_stack(&[0, 0, 0, 0]);

    // --- read from uninitialized memory - address provided as a parameter -----------------------
    let asm_op = format!("{}.{}", asm_op, addr);

    let test = build_op_test!(asm_op, &[5, 6, 7, 8]);
    test.expect_stack(&[0, 0, 0, 0]);

    // --- the rest of the stack is unchanged -----------------------------------------------------

    let test = build_op_test!(asm_op, &[1, 2, 3, 4, 5, 6, 7, 8]);
    test.expect_stack(&[0, 0, 0, 0, 4, 3, 2, 1]);
}

// SAVING STACK VALUES WITHOUT REMOVING THEM (STORE)
// ================================================================================================

#[test]
fn mem_storew() {
    let asm_op = "mem_storew";
    let addr = 0;

    // --- address provided via the stack ---------------------------------------------------------
    let test = build_op_test!(asm_op, &[1, 2, 3, 4, addr]);
    test.expect_stack_and_memory(&[4, 3, 2, 1], addr, &[1, 2, 3, 4]);

    // --- address provided as a parameter --------------------------------------------------------
    let asm_op = format!("{}.{}", asm_op, addr);
    let test = build_op_test!(&asm_op, &[1, 2, 3, 4]);
    test.expect_stack_and_memory(&[4, 3, 2, 1], addr, &[1, 2, 3, 4]);

    // --- the rest of the stack is unchanged -----------------------------------------------------
    let test = build_op_test!(&asm_op, &[0, 1, 2, 3, 4]);
    test.expect_stack_and_memory(&[4, 3, 2, 1, 0], addr, &[1, 2, 3, 4]);
}

// PAIRED OPERATIONS - ABSOLUTE MEMORY (push/pop, pushw/popw, loadw/storew)
// ================================================================================================

#[test]
fn inverse_operations() {
    // --- pop and push are inverse operations, so the stack should be left unchanged -------------
    let source = "
        begin
            push.0
            mem_store
            drop
            mem_store.1
            drop
            push.1
            mem_load
            mem_load.0
        end";

    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    let test = build_test!(source, &inputs);
    test.expect_stack(&final_stack);

    // --- storew and loadw are inverse operations, so the stack should be left unchanged ---------
    let source = "
        begin
            push.0
            mem_storew
            mem_storew.1
            push.1
            mem_loadw
            mem_loadw.0
        end";

    let inputs = [0, 1, 2, 3, 4];
    let mut final_stack = inputs;
    final_stack.reverse();

    let test = build_test!(source, &inputs);
    test.expect_stack(&final_stack);
}

#[test]
fn read_after_write() {
    // --- write to memory first, then test read with push --------------------------------------
    let test = build_op_test!("mem_storew.0 mem_load.0", &[1, 2, 3, 4]);
    test.expect_stack(&[1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with pushw --------------------------------------
    let test = build_op_test!("mem_storew.0 push.0.0.0.0 mem_loadw.0", &[1, 2, 3, 4]);
    test.expect_stack(&[4, 3, 2, 1, 4, 3, 2, 1]);

    // --- write to memory first, then test read with loadw --------------------------------------
    let test = build_op_test!("mem_storew.0 dropw mem_loadw.0", &[1, 2, 3, 4, 5, 6, 7, 8]);
    test.expect_stack(&[8, 7, 6, 5]);
}
