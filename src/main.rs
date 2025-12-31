fn main() {
    // =========================================================
    // CASE 1: Immutable binding → immutable reference (OK)
    // =========================================================

    let x = 10;            // immutable binding
    let a = &x;            // a: &i32 (immutable reference)

    let b = &*a;           // immutable reborrow from immutable ref (OK)
    // Both a and b can coexist (many reads allowed)

    println!("a = {}", a);
    println!("b = {}", b);

    // let c = &mut *a;    // ❌ NOT ALLOWED
    // Cannot create mutable reference from immutable reference


    // =========================================================
    // CASE 2: Mutable binding → mutable reference (OK)
    // =========================================================

    let mut y = 20;        // mutable binding
    let m = &mut y;        // m: &mut i32 (exclusive access)

    *m += 1;               // OK: mutate through mutable reference
    // y is now 21


    // =========================================================
    // CASE 3: Mutable ref → immutable reborrow (OK)
    // =========================================================

    let r = &*m;           // r: &i32 (immutable reborrow)
    // r borrows from m
    // m is now FROZEN while r is active

    println!("r = {}", r);

    // *m += 1;            // ❌ NOT ALLOWED
    // Cannot mutate while immutable reborrow exists


    // =========================================================
    // CASE 4: Immutable reborrow ends → m is unfrozen
    // =========================================================

    // r's last use was above
    // Rust (NLL) now considers r inactive

    *m += 1;               // OK again
    // y is now 22


    // =========================================================
    // CASE 5: Mutable ref → mutable reborrow (OK, but strict)
    // =========================================================

    let r2 = &mut *m;      // r2: &mut i32 (mutable reborrow)
    // r2 becomes the ONLY active mutable reference
    // m is frozen while r2 exists

    *r2 += 1;              // OK
    // y is now 23

    // *m += 1;            // ❌ NOT ALLOWED
    // Two mutable references cannot be active at once


    // =========================================================
    // CASE 6: Mutable reborrow ends → m is unfrozen again
    // =========================================================

    // r2's last use was above

    *m += 1;               // OK
    // y is now 24


    // =========================================================
    // CASE 7: What is NEVER allowed
    // =========================================================

    // let z = &mut y;     // ❌ NOT ALLOWED
    // Cannot create another mutable reference while m exists

    // let i = &y;         // ❌ NOT ALLOWED
    // Cannot create immutable reference from owner while m exists

    println!("final y = {}", y);
}
