// Zeming Lin
// For CS4414, probably generates a deadlock.
// Inspired by David Evans code from class 13 for CS4414, Fall '13
// This works on an AMD cpu running ubuntu LTS 12.04,
// 	but it fails on an Intel Atom netbook.
// 	Rust tasks also don't like to run concurrently on the same netbook
// 	so it probably isn't the code's fault.

type Semaphore = Option<uint> ; // either None (available) or owner

static mut count: uint = 0; // protected by lock
static mut lock1: Semaphore = None; 
static mut lock2: Semaphore = None;

fn grab_lock(id: uint) {
    unsafe {
	println(fmt!("%u is grabbing lock...", id));
	while (lock1.is_some() && lock2.is_some()) {
	    ; // wait for lock 
	}
	if !lock1.is_some() {
	    lock1 = Some(id);
	    print(fmt!("Process %u grabbed lock1!\n", id));
	    while (lock2.is_some()) {
		;
	    }
	    lock2 = Some(id);
	    print(fmt!("Process %u grabbed lock2!\n", id));
	}
	else if !lock2.is_some() {
	    lock2 = Some(id);
	    print(fmt!("Process %u grabbed lock2!\n", id));
	    while (lock1.is_some()) {
		;
	    }
	    lock1 = Some(id);
	    print(fmt!("Process %u grabbed lock1!\n", id));
	}
	else { // Oops, another process grabbed the locks
	    grab_lock(id);
	}
    }
}

fn release_locks() {
    unsafe {
	lock1 = None;
	lock2 = None
    }
}

fn update_count(id: uint) {
    unsafe {
	grab_lock(id);
	count += 1;
	println(fmt!("Count updated by %?: %?", id, count));
	release_locks();
    }
}

fn main() {
    for num in range(0u, 10) {
	do spawn {
	    for _ in range(0u, 1000) {
		update_count(num);
	    }
	}
    }
}

