use crate::pb::Spec;
use image::ImageOutputFormat;

mod photon;
pub use photon::Photon;

pub trait Engine {
    // 对 Engine 按照specs进行一系列有序的处理
    fn apply(&mut self, specs: &[Spec]);

    // 从engine中生成目标图片，注意这里用的是 self， 而非self的引用
    fn generate(self, format: ImageOutputFormat) -> Result<Vec<u8>>;
}

// SpecTransform： 未来如果添加更多的spec， 只需要实现它即可
pub trait SpecTransform<T> {
    // 对图片使用 op 做 transform
    fn transform(&mut self, op: T);
}