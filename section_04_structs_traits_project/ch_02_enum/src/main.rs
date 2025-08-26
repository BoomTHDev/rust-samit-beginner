#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[allow(dead_code)]
fn main_enum() {
    let home = IpAddrKind::V4;
    let loopback = IpAddrKind::V6;

    println!("Home IP kind: {:?}", home);
    println!("Loopback IP kind: {:?}", loopback);

    let localhost = IpAddr::V4(127, 0, 0, 1);
    let google = IpAddr::V6(String::from("::1"));
    println!("Localhost IP: {:?}", localhost);
    println!("Google IP: {:?}", google);

    if let IpAddr::V4(a, b, c, d) = localhost {
        println!("Localhost IP fields: {}.{}.{}.{}", a, b, c, d);
    }

    if let IpAddr::V6(addr) = google {
        println!("Google IP address: {}", addr);
    }
}

#[derive(Debug)]
enum TaskStatus {
    Pending,
    InProgress { assigned_to: String },
    Completed { completed_by: String, duration: u32 },
    Cancelled { reason: String },
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

fn main() {
    // การสร้างตัวอย่าง Task
    let task1 = Task {
        id: 1,
        title: String::from("Implement feature X"),
        status: TaskStatus::Pending,
    };

    let task2 = Task {
        id: 2,
        title: String::from("Fix bug Y"),
        status: TaskStatus::InProgress {
            assigned_to: String::from("John Doe"),
        },
    };

    let task3 = Task {
        id: 3,
        title: String::from("Complete project Z"),
        status: TaskStatus::Completed {
            completed_by: String::from("Jane Smith"),
            duration: 120,
        },
    };

    let task4 = Task {
        id: 4,
        title: String::from("Cancel project W"),
        status: TaskStatus::Cancelled {
            reason: String::from("Insufficient resources"),
        },
    };

    println!("Task 1: {:?}", task1);
    println!("Task 2: {:?}", task2);
    println!("Task 3: {:?}", task3);
    println!("Task 4: {:?}", task4);
}
