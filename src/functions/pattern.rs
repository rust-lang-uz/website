use uuid::Uuid;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PatternProps {
    #[prop_or(40)]
    pub size: i32,
    #[prop_or(16)]
    pub gap_x: i32,
    #[prop_or(8)]
    pub gap_y: i32,
    #[prop_or_else(|| vec![
        vec![0, 1, 0, 1, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 0, 0, 1],
        vec![0, 1, 0, 1, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 0, 0, 1],
    ])]
    pub pattern: Vec<Vec<i32>>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Pattern)]
pub fn pattern(props: &PatternProps) -> Html {
    let id = Uuid::new_v4().to_string();
    let width = props.pattern[0].len() as i32 * props.size
        + (props.pattern[0].len() as i32 - 1) * props.gap_x;
    let height =
        props.pattern.len() as i32 * props.size + (props.pattern.len() as i32 - 1) * props.gap_y;

    html! {
        <svg aria-hidden="true" width={width.to_string()} height={height.to_string()} class={props.class.clone()}>
            <defs>
                <symbol id={format!("{}-0", id)} width={props.size.to_string()} height={props.size.to_string()}>
                    <rect class="fill-orange-500" width={props.size.to_string()} height={props.size.to_string()} />
                    <circle class="fill-orange-600" cx={(props.size / 2).to_string()} cy={(props.size / 2).to_string()} r={(props.size * 13 / 40).to_string()} />
                </symbol>
                <symbol id={format!("{}-1", id)} width={props.size.to_string()} height={props.size.to_string()}>
                    <circle
                        class="fill-orange-300"
                        cx={(props.size / 2).to_string()}
                        cy={(props.size / 2).to_string()}
                        r={(props.size / 2).to_string()}
                    />
                    <rect
                        class="fill-orange-600"
                        width={(props.size / 2).to_string()}
                        height={(props.size / 2).to_string()}
                        x={(props.size / 4).to_string()}
                        y={(props.size / 4).to_string()}
                    />
                </symbol>
            </defs>
            {
                for props.pattern.iter().enumerate().map(|(row_index, row)| {
                    html! {
                        for row.iter().enumerate().map(|(column_index, &shape)| {
                            html! {
                                <use
                                    key={format!("{}-{}", row_index, column_index)}
                                    href={format!("#{}-{}", id, shape)}
                                    x={(column_index as i32 * props.size + column_index as i32 * props.gap_x).to_string()}
                                    y={(row_index as i32 * props.size + row_index as i32 * props.gap_y).to_string()}
                                />
                            }
                        })
                    }
                })
            }
        </svg>
    }
}
