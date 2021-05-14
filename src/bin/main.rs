use neatlib::{network::feedforward::Feedforward, pool::Pool};
use neatlib::{
    network::Network,
    parameters::{MutationParameters, Parameters},
};

// TODO LISTEN
// [cancel] 1. NodeData에 input sum 저장해서 activate 하는거 변경하기 (node data는 stateless한게 좋은 것 같다)
// 2. 메소드 하나씩 보면서 리팩토링 하기
// 3. 모든 메소드에 테스트 케이스 추가하기 (coverage 100%!)

fn main() {
    env_logger::init();

    let params = Parameters {
        input_number: 2,
        output_number: 1,
        population: 150,
        mutation: MutationParameters {
            weight_perturbation: 0.8,
            weight_assign: 0.1,
            add_connection: 0.5,
            remove_connection: 0.5,
            toggle_connection: 0.0,
            add_node: 0.2,
            remove_node: 0.2,

            weight_min: -30.0,
            weight_max: 30.0,

            perturb_min: -1.0,
            perturb_max: 1.0,
        },
    };
    let mut pool = Pool::<Feedforward>::new(params);

    let data = vec![
        (vec![0.0, 0.0], 0.0),
        (vec![0.0, 1.0], 1.0),
        (vec![1.0, 0.0], 1.0),
        (vec![1.0, 1.0], 0.0),
    ];

    pool.evolve(300, 3.9, |networks| {
        for network in networks {
            let mut err = 0.0;

            for (inputs, expected) in &data {
                let output = network.activate(inputs).unwrap()[0];
                err += (output - expected) * (output - expected);
            }

            network.evaluate(4.0 - err);
        }
    });
}
