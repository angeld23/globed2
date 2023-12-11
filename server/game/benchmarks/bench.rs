#![allow(clippy::wildcard_imports)]
use bytebuffer::{ByteBuffer, ByteReader};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use globed_game_server::{data::*, managers::PlayerManager};
use rand::RngCore;

fn buffers(c: &mut Criterion) {
    let data = PlayerAccountData {
        account_id: 234_234_234,
        name: FastString::from_str("hit his is my name"),
        icons: PlayerIconData::default(),
        special_user_data: Some(SpecialUserData {
            name_color: Color3B { r: 10, g: 100, b: 200 },
        }),
    };

    c.bench_function("alloca-byte-buffer", |b| {
        b.iter(black_box(|| {
            alloca::with_alloca(PlayerAccountData::ENCODED_SIZE * 64, |data_| {
                let stackarray = unsafe {
                    let ptr = data_.as_mut_ptr().cast::<u8>();
                    std::slice::from_raw_parts_mut(ptr, std::mem::size_of_val(data_))
                };

                let mut buf = FastByteBuffer::new(stackarray);
                for _ in 0..64 {
                    buf.write_value(&data);
                }

                // let mut buf = ByteReader::from_bytes(stackarray);
                // for _ in 0..64 {
                //     assert_eq!(buf.read_value::<PlayerAccountData>().unwrap().account_id, data.account_id);
                // }
            });
        }));
    });

    c.bench_function("fast-byte-buffer", |b| {
        b.iter(black_box(|| {
            let mut stackarray = [0u8; PlayerAccountData::ENCODED_SIZE * 64];
            let mut buf = FastByteBuffer::new(&mut stackarray);
            for _ in 0..64 {
                buf.write_value(&data);
            }

            // let mut buf = ByteReader::from_bytes(&stackarray);
            // for _ in 0..64 {
            //     assert_eq!(buf.read_value::<PlayerAccountData>().unwrap().account_id, data.account_id);
            // }
        }));
    });

    c.bench_function("slow-byte-buffer", |b| {
        b.iter(black_box(|| {
            let mut buffer = ByteBuffer::new();
            for _ in 0..64 {
                buffer.write_value(&data);
            }
        }));
    });
}

fn structs(c: &mut Criterion) {
    let mut data = [0u8; 2048];
    rand::thread_rng().fill_bytes(&mut data);

    c.bench_function("encode-audio-frame", |b| {
        b.iter(black_box(|| {
            let data = FastEncodedAudioFrame {
                data: data.to_vec().into(),
            };
            let mut stack_array = [0u8; 3000 * 8];
            let mut buf = FastByteBuffer::new(&mut stack_array);
            for _ in 0..8 {
                buf.write_value(&data);
            }

            let written_data = buf.as_bytes();

            for _ in 0..128 {
                let mut reader = ByteReader::from_bytes(written_data);

                let abc = reader.read_value::<FastEncodedAudioFrame>().unwrap();
                assert_eq!(abc.data.len(), written_data.len());
            }
        }));
    });
}

fn managers(c: &mut Criterion) {
    c.bench_function("player-manager", |b| {
        b.iter(black_box(|| {
            let mut manager = PlayerManager::new();

            for level_id in 0..100 {
                for account_id in 0..10 {
                    manager.add_to_level(level_id, level_id * 10 + account_id);
                    manager.set_player_data(
                        level_id * 10 + account_id,
                        &PlayerData {
                            percentage: 0,
                            attempts: 0,
                        },
                    );
                }
            }

            let mut total_players = 0;
            for i in 0..100 {
                let count = manager.get_player_count_on_level(i).unwrap_or(0);
                assert_eq!(count, 10);

                let total = manager.for_each_player_on_level(
                    i,
                    |_, _, p| {
                        *p += 1;
                        true
                    },
                    &mut total_players,
                );

                assert_eq!(total, count);
            }

            assert_eq!(total_players, 1000);

            for level_id in 0..100 {
                for account_id in 0..10 {
                    manager.remove_from_level(level_id, level_id * 10 + account_id);
                    manager.remove_player(level_id * 10 + account_id);
                }
            }
        }));
    });
}

criterion_group!(benches, buffers, structs, managers);
// criterion_group!(benches, structs);
criterion_main!(benches);
