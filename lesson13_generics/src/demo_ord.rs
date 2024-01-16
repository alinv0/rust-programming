#[derive(Ord, Eq, PartialEq, PartialOrd, Copy, Clone, Debug)]
struct ExamMark {
    value: i32,
}

pub fn do_it() {
    println!("In demo_ord::do_it()");

    let m1 = ExamMark { value: 75 };
    let m2 = ExamMark { value: 10 };
    let m3 = ExamMark { value: 90 };
    let m4 = ExamMark { value: -60 };

    println!("m1: {:?}", m1);
    println!("m2: {:?}", m2);
    println!("m3: {:?}", m3);
    println!("m4: {:?}", m4);

    println!("m1.min(m2) {:?}", m1.min(m2));
    println!("m1.max(m2) {:?}", m1.max(m2));
    println!("m3 clamped {:?}", m3.clamp(
        ExamMark { value: 0 }, ExamMark { value: 100 }));
    println!("m4 clamped {:?}", m4.clamp(
        ExamMark { value: 0 }, ExamMark { value: 100 }));
}