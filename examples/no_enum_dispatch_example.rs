// 首先，定义了一个名为 Animal 的 trait，该 trait 有一个方法 make_sound。
trait Animal {
    fn make_sound(&self);
}

// 然后，定义了两个结构体 Dog 和 Cat，并为它们分别实现了 Animal trait。
// 这意味着 Dog 和 Cat 都有 make_sound 方法。
struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Bark!");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}

// 如果不使用 enum_dispatch，我们需要为 Pet 枚举实现 Animal trait，
// 然后在 match 表达式中根据 Pet 的变体调用相应的方法。
enum Pet {
    Dog(Dog),
    Cat(Cat),
}
impl Animal for Pet {
    fn make_sound(&self) {
        match self {
            Pet::Dog(dog) => dog.make_sound(),
            Pet::Cat(cat) => cat.make_sound(),
        }
    }
}

// 最后，在 main 函数中，创建了一个 Pet 枚举的实例 my_pet，并调用了它的 make_sound 方法。
// 由于 my_pet 是 Dog 变体，所以输出的是 "Bark!"
fn main() {
    let my_pet: Pet = Pet::Dog(Dog);
    my_pet.make_sound(); // 输出 "Bark!"
    let my_pet2: Pet = Pet::Cat(Cat);
    my_pet2.make_sound(); // 输出 "Meow!"
}
