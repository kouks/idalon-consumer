use error::IdalonError;
use models::{
    generic::Model,
    night::{Night, NightFilters},
};
use tokio_stream::StreamExt;

pub mod error;
pub mod models;

#[tokio::main]
pub async fn main() -> Result<(), crate::error::IdalonError> {
    let mut paginator = Night::paginate(NightFilters::leaderboard());

    while let Some(result) = paginator.next().await {
        if result.is_err() {
            return Err(IdalonError::new("Failed to fetch page."));
        }

        let page = result.expect("Result is not None as checked before.");

        tokio::spawn(async move {
            // for night in page.items {
            //     println!("{:?}", night)
            // }

            println!("Median is {:.3}", find_median(&page.items));
            println!("Average is {:.3}", find_average(&page.items));
        });
    }

    // println!("Median is {:.3}", find_median(&data.items));
    // println!("Average is {:.3}", find_average(&data.items));

    let night = Night::find_one("d4bad0ba-5b5a-412b-a75a-e92e24c4f908").await;

    println!("Night: {:?}", night);

    Ok(())
}

fn find_median(nights: &Vec<Night>) -> f32 {
    let median_index = nights.len() / 2;

    if median_index % 2 == 0 {
        return find_average(&nights[(median_index - 1)..(median_index + 1)].to_vec());
    }

    nights[median_index].average_real_time
}

fn find_average(nights: &Vec<Night>) -> f32 {
    let total_time = nights
        .into_iter()
        .fold(0f32, |carry, item| carry + item.average_real_time);

    let total_items = nights.len() as f32;

    if total_items == 0.0 {
        return 0f32;
    }

    return total_time / total_items;
}

#[tokio::test]
async fn it_finds_median() {
    let nights = vec![
        Night {
            uuid: String::from("1"),
            average_real_time: 1f32,
        },
        Night {
            uuid: String::from("2"),
            average_real_time: 2f32,
        },
        Night {
            uuid: String::from("3"),
            average_real_time: 2f32,
        },
        Night {
            uuid: String::from("4"),
            average_real_time: 2f32,
        },
        Night {
            uuid: String::from("5"),
            average_real_time: 2f32,
        },
        Night {
            uuid: String::from("6"),
            average_real_time: 10f32,
        },
    ];

    assert_eq!(find_median(&nights), 2f32);
}
