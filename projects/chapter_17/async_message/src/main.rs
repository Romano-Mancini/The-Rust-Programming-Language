use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx1 = tx.clone();

        let vals1 = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        let vals2 = vec![
            String::from("hi 2"),
            String::from("from 2"),
            String::from("the 2"),
            String::from("future 2"),
        ];

        let fut1 = async move {
            for val in vals1 {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async move {
            for val in vals2 {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut3 = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join!(fut1, fut2, fut3);
    });
}
