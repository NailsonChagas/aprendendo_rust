use rand as r;
use lazy_static::lazy_static;

mod example;
use example::sub;
use example::sub2;
use example::teste::fundo;

fn main() {
    lazy_static!{};
    r::random::<u8>();
    sub::teste();
    sub2::testando();
    fundo::testand();
}
