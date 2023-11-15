use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct StarIconProps {
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(StarIcon)]
pub fn star_icon(props: &StarIconProps) -> Html {
    html! {
        <svg aria-hidden="true" viewBox="0 0 20 20" class={classes!(props.class.clone())}>
            <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct StarRatingProps {
    #[prop_or(5)]
    pub rating: usize,
}

#[function_component(StarRating)]
pub fn star_rating(props: &StarRatingProps) -> Html {
    html! {
        <div class="flex gap-1">
            {
                (0..props.rating).map(|_index| {
                    html! { <StarIcon class="h-5 w-5 fill-current" /> }
                }).collect::<Html>()
            }
        </div>
    }
}
