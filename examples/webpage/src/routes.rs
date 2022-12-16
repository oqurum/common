use yew::prelude::*;
use yew_router::{prelude::Link, Routable};

use crate::components::*;

#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/")]
    Home,

    #[at("/multiselect")]
    MultiSelect,

    #[at("/expandingtext")]
    ExpandableContainer,

    #[at("/carousel")]
    Carousel,

    #[at("/filesearch")]
    FileSearch,

    #[at("/filter")]
    Filter,

    #[at("/popups")]
    Popups,
}

#[allow(clippy::let_unit_value)]
pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::MultiSelect => html! { <MultiSelectPage /> },
        AppRoute::ExpandableContainer => html! { <ExpandableContainerPage /> },
        AppRoute::Carousel => html! { <CarouselPage /> },
        AppRoute::FileSearch => html! { <FileSearchPage /> },
        AppRoute::Filter => html! { <FilterPage /> },
        AppRoute::Popups => html! { <PopupsPage /> },
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <ul>
            <li><Link<AppRoute> to={ AppRoute::MultiSelect }>{ "MultiSelect" }</Link<AppRoute>></li>
            <li><Link<AppRoute> to={ AppRoute::ExpandableContainer }>{ "Expandable Container" }</Link<AppRoute>></li>
            <li><Link<AppRoute> to={ AppRoute::Carousel }>{ "Carousel" }</Link<AppRoute>></li>
            <li><Link<AppRoute> to={ AppRoute::FileSearch }>{ "File Search" }</Link<AppRoute>></li>
            <li><Link<AppRoute> to={ AppRoute::Filter }>{ "Filter" }</Link<AppRoute>></li>
            <li><Link<AppRoute> to={ AppRoute::Popups }>{ "Popups" }</Link<AppRoute>></li>
        </ul>
    }
}
