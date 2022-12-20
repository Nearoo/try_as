#[cfg(test)]
mod test {

    use macros;
    use traits::*;

    #[test]
    fn test_macros() {
        #[derive(macros::From, macros::TryInto, macros::TryAsRef, Debug)]
        enum Hello {
            Foo(u32),
            Bar(f32),
        }

        let hello = Hello::from(0);

        let x1: u32 = hello.unwrap_into();
        println!("Hello: {:?}", x1);
    }

    #[test]
    fn test_derive_from() {
        #[derive(macros::From, PartialEq, Eq, Debug)]
        enum Data {
            U8(u8),
            String(String),
        }

        let num = Data::from(2);
        let num_2: Data = 2.into();
        assert_eq!(num, Data::U8(2));
        assert_eq!(num_2, Data::U8(2));
    }

    #[test]
    fn test_try_into() {
        #[derive(macros::TryInto, Clone, Debug, PartialEq, Eq)]
        enum Data {
            U8(u8),
            String(String),
        }

        let num = Data::U8(22);
        let u8: Result<u8, Data> = num.clone().try_into();
        let str: Result<String, Data> = num.clone().try_into();
        assert!(u8.is_ok());
        assert!(str.is_err());
        assert_eq!(str.unwrap_err(), num);
    }

    #[test]
    fn test_try_as_ref() {
        #[derive(macros::TryAsRef)]
        enum Data {
            U8(u8),
            String(String),
        }

        let num = Data::U8(2);

        let u8: Option<&u8> = num.try_as_ref();
        assert!(u8.is_some());
        assert!(*u8.unwrap() == 2);

        let str: Option<&String> = num.try_as_ref();
        assert!(str.is_none());
    }

    #[test]
    fn test_try_as_mut() {
        #[derive(macros::TryAsMut)]
        enum Data {
            U8(u8),
            String(String),
        }

        let mut num = Data::U8(2);

        let u8: Option<&mut u8> = num.try_as_mut();
        assert!(u8.is_some());
        assert!(*u8.unwrap() == 2);

        let str: Option<&mut String> = num.try_as_mut();
        assert!(str.is_none());
    }
}
