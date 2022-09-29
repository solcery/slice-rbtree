use borsh::{BorshDeserialize, BorshSerialize};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use slice_rbtree::{tree_size, RBTree};
use std::collections::BTreeMap;

const SIZES: [u32; 8] = [10, 20, 40, 80, 160, 320, 640, 1280];

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone, Copy, PartialEq)]
struct MyType {
    array: [u8; 10],
    float: f64,
    num: u64,
    num2: u32,
}

impl MyType {
    fn gen(i: u8) -> Self {
        let i6 = u64::from(i);
        let i3 = u32::from(i);
        MyType {
            array: [i; 10],
            float: f64::from(i) * 1.25,
            num: i6 * i6 * i6 + i6 * i6 + i6,
            num2: 5 * i3 + 8 * i3 * i3,
        }
    }
}

fn access_one_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("Access one value");

    let mut map_buffer = vec![
        0u8;
        tree_size(
            std::mem::size_of::<u32>(),
            std::mem::size_of::<MyType>(),
            2000
        )
    ];

    let expected_value = MyType::gen(3);

    for i in SIZES {
        group.bench_with_input(BenchmarkId::new("BTreeMap", i), &i, |b, i| {
            let map = (0..*i)
                .map(|i: u32| (i as u32, MyType::gen(i.to_le_bytes()[0])))
                .collect::<BTreeMap<u32, MyType>>();

            map.serialize(&mut map_buffer.as_mut_slice()).unwrap();

            drop(map);
            let map = BTreeMap::<u32, MyType>::deserialize(&mut black_box(map_buffer.as_slice()))
                .unwrap();
            b.iter(|| assert_eq!(map.get(&3), Some(&expected_value)))
        });
        group.bench_with_input(BenchmarkId::new("RBTree", i), &i, |b, i| {
            let mut slice_map = RBTree::<
                u32,
                MyType,
                { std::mem::size_of::<u32>() },
                { std::mem::size_of::<MyType>() },
            >::init_slice(map_buffer.as_mut_slice())
            .unwrap();

            for j in 0..*i {
                slice_map
                    .insert(j, MyType::gen(j.to_le_bytes()[0]))
                    .unwrap();
            }
            drop(slice_map);
            let map = unsafe {
                RBTree::<
                    u32,
                    MyType,
                    { std::mem::size_of::<u32>() },
                    { std::mem::size_of::<MyType>() },
                >::from_slice(black_box(map_buffer.as_mut_slice()))
                .unwrap()
            };
            b.iter(|| assert_eq!(map.get(&3), Some(expected_value.clone())))
        });
    }
    group.finish();
}

fn deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deserialization");

    let mut map_buffer = vec![
        0u8;
        tree_size(
            std::mem::size_of::<u32>(),
            std::mem::size_of::<MyType>(),
            2000
        )
    ];

    for i in SIZES {
        group.bench_with_input(BenchmarkId::new("BTreeMap", i), &i, |b, i| {
            let map = (0..*i)
                .map(|i: u32| (i as u32, MyType::gen(i.to_le_bytes()[0])))
                .collect::<BTreeMap<u32, MyType>>();

            map.serialize(&mut map_buffer.as_mut_slice()).unwrap();

            drop(map);
            b.iter(|| {
                let map =
                    BTreeMap::<u32, MyType>::deserialize(&mut black_box(map_buffer.as_slice()))
                        .unwrap();
                assert!(!map.is_empty());
                map
            })
        });
        group.bench_with_input(BenchmarkId::new("RBTree", i), &i, |b, i| {
            let mut slice_map = RBTree::<
                u32,
                MyType,
                { std::mem::size_of::<u32>() },
                { std::mem::size_of::<MyType>() },
            >::init_slice(map_buffer.as_mut_slice())
            .unwrap();

            for j in 0..*i {
                slice_map
                    .insert(j, MyType::gen(j.to_le_bytes()[0]))
                    .unwrap();
            }
            drop(slice_map);
            b.iter(|| {
                let map = unsafe {
                    RBTree::<
                        u32,
                        MyType,
                        { std::mem::size_of::<u32>() },
                        { std::mem::size_of::<MyType>() },
                    >::from_slice(black_box(map_buffer.as_mut_slice()))
                    .unwrap()
                };
                assert!(!map.is_empty());
            })
        });
    }
    group.finish();
}

fn add_one_value(c: &mut Criterion) {
    let mut group = c.benchmark_group("Add one value");

    let mut map_buffer = vec![
        0u8;
        tree_size(
            std::mem::size_of::<u32>(),
            std::mem::size_of::<MyType>(),
            2000
        )
    ];

    for i in SIZES {
        group.bench_with_input(BenchmarkId::new("BTreeMap", i), &i, |b, i| {
            let map = (0..*i)
                .map(|i: u32| (i as u32, MyType::gen(i.to_le_bytes()[0])))
                .collect::<BTreeMap<u32, MyType>>();

            map.serialize(&mut map_buffer.as_mut_slice()).unwrap();

            drop(map);
            let mut map =
                BTreeMap::<u32, MyType>::deserialize(&mut black_box(map_buffer.as_slice()))
                    .unwrap();
            b.iter(|| {
                assert_eq!(map.insert(2048, MyType::gen(208)), None);
                map.remove(&2048)
            })
        });
        group.bench_with_input(BenchmarkId::new("RBTree", i), &i, |b, i| {
            let mut slice_map = RBTree::<
                u32,
                MyType,
                { std::mem::size_of::<u32>() },
                { std::mem::size_of::<MyType>() },
            >::init_slice(map_buffer.as_mut_slice())
            .unwrap();

            for j in 0..*i {
                slice_map
                    .insert(j, MyType::gen(j.to_le_bytes()[0]))
                    .unwrap();
            }
            drop(slice_map);
            let mut map = unsafe {
                RBTree::<
                    u32,
                    MyType,
                    { std::mem::size_of::<u32>() },
                    { std::mem::size_of::<MyType>() },
                >::from_slice(black_box(map_buffer.as_mut_slice()))
                .unwrap()
            };
            b.iter(|| {
                assert_eq!(map.insert(2048, MyType::gen(208)), Ok(None));
                map.remove(&2048)
            })
        });
    }
    group.finish();
}

criterion_group!(benches, deserialization, access_one_value, add_one_value);
criterion_main!(benches);
