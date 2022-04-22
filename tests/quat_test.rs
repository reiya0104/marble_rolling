extern crate rstest;
#[cfg(test)]
extern crate speculate;

use rstest::*;
use speculate::speculate;

use bevy::prelude::*;

speculate! {
    describe "四元数のテスト" {
        #[rstest]
        fn quat_test() {
            // Quat(x, y, z, w) = w + x i + y j + z k
            // 回転は q * a * q.conjugate()

            let normal_vector = Quat::from_xyzw(0.0, 1.0, 0.0, 0.0);
            let quat = Quat::from_xyzw(0.27433386, 0.0, 0.26681, 0.9238795);
            println!("Quat::IDENTITY = {:?}", Quat::IDENTITY);
            // quat はすでに正規化されている
            println!("g = {:?}, abs: {:?}", quat, quat.length());
            println!("{:?}", rotate(quat, normal_vector));
            // Vec3(-0.49300057, 0.7071067, 0.5069029)
            // Quat(-0.49300057, 0.7071067, 0.5069029, 0.0)

        }
    }
}

fn rotate(q1: Quat, q2: Quat) -> Quat {
    q1 * q2 * q1.conjugate()
}
