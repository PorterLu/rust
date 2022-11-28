# Rust Copy和Clone的区别

## Copy

​	`Copy`的全名是`std::marker::Copy` 。在这个模块中全是特殊的，目前稳定的是4个，分别是`Copy`、`Send`、`Sized`和`Sync`。它们的特殊之处在于这些`trait`是跟编译器进行绑定的，如果一个类型实现了`Copy`这个`trait`，那么编译器就会认为按字节拷贝是不会产生任何问题的，那么它在变量的绑定、参数的传递，函数值返回的情况下，将不再有`move`语义，而是`Copy`的语义。

​	`Copy`的实现是有条件的，对于自定义的类型，只有所有的成员都实现了`Copy`那么这个类型才能实现`Copy` 。对于数字、布尔、不可变引用类型，都具有`Copy`属性类型。但是`Box`, `Vec`和可变借用等则不具备`Copy`属性。对于数组，如果数组的元素是`Copy`的，那么这个数组是可以`Copy`的，对于元组也是如此。

## Clone

​	`Clone`的全名是`std::clone::Clone`，它的完整声明如下：

```rust
pub trait Clone : Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self){
        *self = source.clone()
    }
}
```

​	这里`Clone`是交给我们进行实现的，不同类型的`clone`的语义可能不同，但是有一点是确定的，如果实现了`Copy`的类型，它的`Clone`方法语义应该和`Copy`保持一致。

## 自动derive

我们可以添加如下的代码：

```rust
#[derive(Copy, Clone)]
```

