use neatlib::network::Network;
use neatlib::{network::feedforward::Feedforward, pool::Pool};

// TODO LISTEN
// [cancel] 1. NodeData에 input sum 저장해서 activate 하는거 변경하기 (node data는 stateless한게 좋은 것 같다)
// 2. 메소드 하나씩 보면서 리팩토링 하기
// 3. 모든 메소드에 테스트 케이스 추가하기 (coverage 100%!)

fn main() {
    let input_number = 2;
    let output_number = 1;

    let mut pool = Pool::<Feedforward>::new(input_number, output_number, 150);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    pool.evolve(300, |networks| {
        for network in networks {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected).powf(2.0);
            }

            network.evaluate(4.0 - err);

            if network.fitness().unwrap() >= 3.9 {
                return;
            }
        }
    });
}
