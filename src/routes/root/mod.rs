use axum::{Router, routing};
use maud::{PreEscaped, html};

use crate::{routes::layout, state::State};

pub mod partials;

pub fn routes() -> Router<State> {
    Router::new()
        .route("/", routing::get(page))
        .nest("/partial", partials::routes())
}

#[axum::debug_handler]
async fn page() -> PreEscaped<String> {
    layout(html! {
        div class="w-full p-5 border-b border-zinc-500 flex flex-row gap-6 items-center" {
            h1 class="text-5xl text-amber-500 tracking-tight font-bold" {
                "Trip Maker"
            }
            h2 class="text-2xl tracking-tight font-bold text-zinc-500" {
                (env!("CARGO_PKG_VERSION"))
            }
        }
        div class="w-full p-4" {
            form class="flex flex-col gap-4" id="settings-form" hx-post="/partial/generate" hx-target="#generated" hx-swap="outerHTML" {
                div class="border border-zinc-500 rounded-xl px-2 py-1" {
                    div class="flex flex-row flex-wrap gap-4" {
                        div class="flex flex-col gap-2 flex-1" {
                            div class="flex flex-row gap-2 items-center" {
                                label for="latitude" {
                                    "Latitude"
                                }
                                input name="latitude" id="latitude" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="0" inputmode="decimal" hx-get="/partial/check" hx-target="#latitude-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="latitude-error" {}

                            div class="flex flex-row gap-2 items-center" {
                                label for="longitude" {
                                    "Longitude"
                                }
                                input name="longitude" id="longitude" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="0" inputmode="decimal" hx-get="/partial/check" hx-target="#longitude-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="longitude-error" {}
                            button type="button" class="bg-zinc-700 px-2 py-1 rounded-lg hover:bg-zinc-800 self-start" onclick="setPosition()" {
                                "Current Position"
                            }
                            span class="text-red-500" id="set-position-error" {}
                            button type="button" class="bg-zinc-700 px-2 py-1 rounded-lg hover:bg-zinc-800 self-start" {
                                "Set Address"
                            }
                        }
                        div class="flex flex-col gap-2 flex-1" {
                            div class="flex flex-row gap-2 items-center" {
                                label for="distance_unit" {
                                    "Distance Unit"
                                }
                                select name="distance_unit" class="bg-zinc-700 px-1 py-0.5 rounded-lg" {
                                    option value="Centimeters" {
                                        "Centimeters"
                                    }
                                    option value="Meters" {
                                        "Meters"
                                    }
                                    option value="Kilometers"{
                                        "Kilometers"
                                    }
                                    option value="Inches" {
                                        "Inches"
                                    }
                                    option value="Feet" {
                                        "Feet"
                                    }
                                    option value="Yards" {
                                        "Yards"
                                    }
                                    option value="Miles" selected {
                                        "Miles"
                                    }
                                }
                            }
                            div class="flex flex-row gap-2 items-center" {
                                label for="max_distance" {
                                    "Max Distance"
                                }
                                input id="max_distance" name="max_distance" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="10" inputmode="decimal" hx-get="/partial/check" hx-target="#max-distance-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="max-distance-error" {}
                        }
                        div class="flex flex-col gap-2 flex-1" {
                            div class="flex flex-row gap-2 items-center" {
                                label for="closeness_bias" {
                                    "Closeness Bias"
                                }
                                input id="closeness_bias" name="closeness_bias" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="1" inputmode="decimal" hx-get="/partial/check" hx-target="#closeness-bias-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="closeness-bias-error" {}
                            div class="flex flex-row gap-2 items-center" {
                                label for="minimum_rating" {
                                    "Minimum Rating"
                                }
                                input id="minimum_rating" name="minimum_rating" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="0" inputmode="decimal" hx-get="/partial/check" hx-target="#minimum-rating-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="minimum-rating-error" {}
                            div class="flex flex-row gap-2 items-center" {
                                label for="number_to_generate" {
                                    "Number To Generate"
                                }
                                input id="number_to_generate" name="number_to_generate" class="bg-zinc-700 px-1 py-0.5 rounded-lg" value="5" inputmode="numeric" hx-get="/partial/check" hx-target="#number-to-generate-error" hx-trigger="keyup changed" hx-swap="innerHTML";
                            }
                            span class="text-red-500" id="number-to-generate-error" {}
                        }
                    }
                }
                button type="submit" class="bg-zinc-700 px-2 py-1 rounded-lg text-lg font-bold tracking-tight self-start hover:bg-zinc-800 hover:text-amber-500" {
                    "Generate"
                }
            }
            script {
                (PreEscaped(include_str!("set_location.js")))
            }
        }
        div id="generated" {

        }
    })
}
