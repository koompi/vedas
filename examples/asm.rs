#[macro_export]
macro_rules! f {
    ($v:vis $n:ident => $r:ty) => {
        $v fn $n() -> $r {
            println!("Hello");
        }
    };

    ($a:ident $n:ident => $r:ty) => {
        $a fn $n() -> $r {
            println!("Hello");
        }
    };

    ($v:vis $a:ident $n:ident, $s:ident, $($args:ident: $typ:ty),* => $r:ty) => {
        $v $a fn $n($s, $($args: $typ),* ) -> $r {
            Ok(println!("Hello"))
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), tokio::io::Error> {
    let a = App {
        name: "brilliant".to_string(),
    };
    a.hello("Test".to_string()).await
}

pub struct App {
    name: String,
}

impl App {
    f!(pub async hello, self, name: String => Result<(),std::io::Error>);
}
