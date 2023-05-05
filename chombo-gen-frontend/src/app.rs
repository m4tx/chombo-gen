use yew::prelude::*;

use crate::hand_generator::HandGenerator;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <nav class="navbar navbar-expand-md navbar-dark bg-dark mb-4">
                <div class="container">
                    <a class="navbar-brand" href="#">{ "ChomboGen" }</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarCollapse" aria-controls="navbarCollapse" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarCollapse">
                        <ul class="navbar-nav ms-auto mb-2 mb-md-0">
                            <li class="nav-item">
                                <a class="nav-link" href="https://chombo.club">{ "Kraków Chombo Club" }</a>
                            </li>
                            <li class="nav-item">
                                <a class="nav-link" href="https://github.com/m4tx/chombo-gen">{ "Source" }</a>
                            </li>
                        </ul>
                    </div>
                </div>
            </nav>

            <main class="container">
                <div class="bg-body-tertiary p-5 rounded">
                    <HandGenerator />
                </div>

                <div class="text-center mt-4 text-secondary"><small>
                    { "This uses "}<a href="https://github.com/FluffyStuff/riichi-mahjong-tiles">{ "riichi-mahjong-tiles by FluffyStuff"}</a>{" (licensed under CC BY 4.0), and mahjong tiles by "}<a href="https://www.martinpersson.org/">{ "Martin Persson" }</a>{ "." }
                </small></div>
            </main>
        </>
    }
}
