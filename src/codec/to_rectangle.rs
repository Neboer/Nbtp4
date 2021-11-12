use crate::definition::RetSize;

// 计算面积大于目标且为距离目标整数最近的矩形。
pub fn find_rectangle(target: u32, max_size: RetSize, min_size: RetSize) -> RetSize {
    let x1 = min_size.width as f32;
    let x2 = max_size.width as f32;
    let y1 = min_size.height as f32;
    let y2 = max_size.height as f32;
    let a = target as f32;
    let rx =  (x2*y1 - x1*y2 + (-4.0*(a*x1 - a*x2)*(-y1 + y2) + (x2*y1 - x1*y2)*(x2*y1 - x1*y2)).sqrt())/(2.0*(y1 - y2));
    let ry = a/rx;
    RetSize {
        width: rx.ceil() as u32,
        height: ry.ceil() as u32
    }
}

#[test]
fn test_find_rectangle() {
    let targets: Vec<u32> = (114514u32..116324u32).collect();
    let max_size = RetSize {
        width: 20u32,
        height: 20u32,
    };
    let min_size = RetSize {
        width: 400u32,
        height: 400u32,
    };
    let mut result = Vec::new();
    for tr in &targets {
        result.push(find_rectangle(*tr, max_size, min_size));
    }
    let mut deviations = Vec::new();
    for (index, size) in result.iter().enumerate() {
        deviations.push(size.area() - targets.get(index).unwrap());
    }
    println!("{:?}", &deviations);
}
