use crate::support::Pair;
pub(crate) fn init() -> crate::support::Result<()> {
    static INIT_ONCE: std::sync::OnceLock<crate::support::Result<()>> = std::sync::OnceLock::new();
    INIT_ONCE
        .get_or_init(|| {
            let tau__25: f64 = 6.283185307179586f64;
            let pi__26: f64 = 3.141592653589793f64;
            Ok(())
        })
        .clone()
}
pub trait ShapeTrait:
    crate::support::AsAnyValue + crate::support::AnyValueTrait + std::marker::Send + std::marker::Sync
{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn to_string(&self) -> std::sync::Arc<String>;
}
#[derive(Clone)]
pub struct Shape(pub(crate) std::sync::Arc<dyn ShapeTrait>);
impl Shape {
    pub fn new(selfish: impl ShapeTrait + 'static) -> Shape {
        Shape(std::sync::Arc::new(selfish))
    }
}
crate::impl_any_value_trait_for_interface!(Shape);
impl std::ops::Deref for Shape {
    type Target = dyn ShapeTrait;
    fn deref(&self) -> &Self::Target {
        &(*self.0)
    }
}
pub(crate) struct RectangleStruct {
    width: f64,
    height: f64,
}
#[derive(Clone)]
pub struct Rectangle(pub(crate) std::sync::Arc<std::sync::RwLock<RectangleStruct>>);
impl Rectangle {
    pub fn area(&self) -> f64 {
        return self.0.read().unwrap().width * self.0.read().unwrap().height;
    }
    pub fn perimeter(&self) -> f64 {
        return 2.0f64 * (self.0.read().unwrap().width + self.0.read().unwrap().height);
    }
    pub fn to_string(&self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!(
            "Rectangle of size {} x {}",
            crate::support::float64::to_string(self.0.read().unwrap().width),
            crate::support::float64::to_string(self.0.read().unwrap().height)
        ));
    }
    pub fn new(width__43: f64, height__44: f64) -> Rectangle {
        let width;
        let height;
        width = width__43;
        height = height__44;
        let selfish = Rectangle(std::sync::Arc::new(std::sync::RwLock::new(
            RectangleStruct { width, height },
        )));
        return selfish;
    }
    pub fn width(&self) -> f64 {
        return self.0.read().unwrap().width;
    }
    pub fn height(&self) -> f64 {
        return self.0.read().unwrap().height;
    }
}
impl ShapeTrait for Rectangle {
    fn area(&self) -> f64 {
        self.area()
    }
    fn perimeter(&self) -> f64 {
        self.perimeter()
    }
    fn to_string(&self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
crate::impl_any_value_trait!(Rectangle, [Shape]);
pub(crate) struct CircleStruct {
    radius: f64,
}
#[derive(Clone)]
pub struct Circle(pub(crate) std::sync::Arc<std::sync::RwLock<CircleStruct>>);
impl Circle {
    pub fn area(&self) -> f64 {
        return 3.141592653589793f64
            * self.0.read().unwrap().radius
            * self.0.read().unwrap().radius;
    }
    pub fn perimeter(&self) -> f64 {
        return 6.283185307179586f64 * self.0.read().unwrap().radius;
    }
    pub fn to_string(&self) -> std::sync::Arc<String> {
        return std::sync::Arc::new(format!(
            "Circle of radius {}",
            crate::support::float64::to_string(self.0.read().unwrap().radius)
        ));
    }
    pub fn new(radius__53: f64) -> Circle {
        let radius;
        radius = radius__53;
        let selfish = Circle(std::sync::Arc::new(std::sync::RwLock::new(CircleStruct {
            radius,
        })));
        return selfish;
    }
    pub fn radius(&self) -> f64 {
        return self.0.read().unwrap().radius;
    }
}
impl ShapeTrait for Circle {
    fn area(&self) -> f64 {
        self.area()
    }
    fn perimeter(&self) -> f64 {
        self.perimeter()
    }
    fn to_string(&self) -> std::sync::Arc<String> {
        self.to_string()
    }
}
crate::impl_any_value_trait!(Circle, [Shape]);
pub fn squared_extent(shape__56: Shape) -> f64 {
    let return__24: f64;
    let mut t___177: f64;
    let mut t___178: f64;
    let mut t___114: Circle;
    let mut t___115: Rectangle;
    if crate::support::is::<Circle>(shape__56.clone()) {
        t___114 = crate::support::cast::<Circle>(shape__56.clone()).unwrap();
        t___177 = t___114.radius();
        t___178 = t___114.radius();
        return__24 = t___177 * t___178;
    } else {
        if crate::support::is::<Rectangle>(shape__56.clone()) {
            t___115 = crate::support::cast::<Rectangle>(shape__56.clone()).unwrap();
            let width__58: f64 = t___115.width();
            let height__59: f64 = t___115.height();
            return__24 = crate::support::float64::div(
                width__58 * width__58 + height__59 * height__59,
                4.0f64,
            )
            .unwrap();
        } else {
            return__24 = f64::NAN;
        }
    }
    return return__24;
}
