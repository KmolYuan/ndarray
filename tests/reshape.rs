use ndarray::prelude::*;

use itertools::enumerate;

use ndarray::Order;

#[test]
fn reshape() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let u = v.into_shape((3, 3));
    assert!(u.is_err());
    let u = v.into_shape((2, 2, 2));
    assert!(u.is_ok());
    let u = u.unwrap();
    assert_eq!(u.shape(), &[2, 2, 2]);
    let s = u.into_shape((4, 2)).unwrap();
    assert_eq!(s.shape(), &[4, 2]);
    assert_eq!(s, aview2(&[[1, 2], [3, 4], [5, 6], [7, 8]]));
}

#[test]
#[should_panic(expected = "IncompatibleShape")]
fn reshape_error1() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let _u = v.into_shape((2, 5)).unwrap();
}

#[test]
#[should_panic(expected = "IncompatibleLayout")]
fn reshape_error2() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let mut u = v.into_shape((2, 2, 2)).unwrap();
    u.swap_axes(0, 1);
    let _s = u.into_shape((2, 4)).unwrap();
}

#[test]
fn reshape_f() {
    let mut u = Array::zeros((3, 4).f());
    for (i, elt) in enumerate(u.as_slice_memory_order_mut().unwrap()) {
        *elt = i as i32;
    }
    let v = u.view();
    println!("{:?}", v);

    // noop ok
    let v2 = v.into_shape((3, 4));
    assert!(v2.is_ok());
    assert_eq!(v, v2.unwrap());

    let u = v.into_shape((3, 2, 2));
    assert!(u.is_ok());
    let u = u.unwrap();
    println!("{:?}", u);
    assert_eq!(u.shape(), &[3, 2, 2]);
    let s = u.into_shape((4, 3)).unwrap();
    println!("{:?}", s);
    assert_eq!(s.shape(), &[4, 3]);
    assert_eq!(s, aview2(&[[0, 4, 8], [1, 5, 9], [2, 6, 10], [3, 7, 11]]));
}


#[test]
fn to_shape_easy() {
    // 1D -> C -> C
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let u = v.to_shape(((3, 3), Order::RowMajor));
    assert!(u.is_err());

    let u = v.to_shape(((2, 2, 2), Order::C));
    assert!(u.is_ok());

    let u = u.unwrap();
    assert!(u.is_view());
    assert_eq!(u.shape(), &[2, 2, 2]);
    assert_eq!(u, array![[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);

    let s = u.to_shape((4, 2)).unwrap();
    assert_eq!(s.shape(), &[4, 2]);
    assert_eq!(s, aview2(&[[1, 2], [3, 4], [5, 6], [7, 8]]));

    // 1D -> F -> F
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let u = v.to_shape(((3, 3), Order::ColumnMajor));
    assert!(u.is_err());

    let u = v.to_shape(((2, 2, 2), Order::ColumnMajor));
    assert!(u.is_ok());

    let u = u.unwrap();
    assert!(u.is_view());
    assert_eq!(u.shape(), &[2, 2, 2]);
    assert_eq!(u, array![[[1, 5], [3, 7]], [[2, 6], [4, 8]]]);

    let s = u.to_shape(((4, 2), Order::ColumnMajor)).unwrap();
    assert_eq!(s.shape(), &[4, 2]);
    assert_eq!(s, array![[1, 5], [2, 6], [3, 7], [4, 8]]);
}

#[test]
fn to_shape_copy() {
    // 1D -> C -> F
    let v = ArrayView::from(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let u = v.to_shape(((4, 2), Order::RowMajor)).unwrap();
    assert_eq!(u.shape(), &[4, 2]);
    assert_eq!(u, array![[1, 2], [3, 4], [5, 6], [7, 8]]);

    let u = u.to_shape(((2, 4), Order::ColumnMajor)).unwrap();
    assert_eq!(u.shape(), &[2, 4]);
    assert_eq!(u, array![[1, 5, 2, 6], [3, 7, 4, 8]]);

    // 1D -> F -> C
    let v = ArrayView::from(&[1, 2, 3, 4, 5, 6, 7, 8]);
    let u = v.to_shape(((4, 2), Order::ColumnMajor)).unwrap();
    assert_eq!(u.shape(), &[4, 2]);
    assert_eq!(u, array![[1, 5], [2, 6], [3, 7], [4, 8]]);

    let u = u.to_shape((2, 4)).unwrap();
    assert_eq!(u.shape(), &[2, 4]);
    assert_eq!(u, array![[1, 5, 2, 6], [3, 7, 4, 8]]);
}

#[test]
fn to_shape_add_axis() {
    // 1D -> C -> C
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let u = v.to_shape(((4, 2), Order::RowMajor)).unwrap();

    assert!(u.to_shape(((1, 4, 2), Order::RowMajor)).unwrap().is_view());
    assert!(u.to_shape(((1, 4, 2), Order::ColumnMajor)).unwrap().is_owned());
}


#[test]
fn to_shape_copy_stride() {
    let v = array![[1, 2, 3, 4], [5, 6, 7, 8]];
    let vs = v.slice(s![.., ..3]);
    let lin1 = vs.to_shape(6).unwrap();
    assert_eq!(lin1, array![1, 2, 3, 5, 6, 7]);
    assert!(lin1.is_owned());

    let lin2 = vs.to_shape((6, Order::ColumnMajor)).unwrap();
    assert_eq!(lin2, array![1, 5, 2, 6, 3, 7]);
    assert!(lin2.is_owned());
}

#[test]
#[should_panic(expected = "IncompatibleShape")]
fn to_shape_error1() {
    let data = [1, 2, 3, 4, 5, 6, 7, 8];
    let v = aview1(&data);
    let _u = v.to_shape((2, 5)).unwrap();
}