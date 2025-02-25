# Yew Examples

## How to run

The examples are built with [trunk](https://github.com/thedodd/trunk).
You can install it with the following command:

```bash
# at some point in the future, trunk will automatically download wasm-bindgen
cargo install trunk wasm-bindgen-cli
```

Running an example is as easy as running a single command:

```bash
# move into the directory of the example you want to run
# In this case it's the todomvc example
cd examples/todomvc

# build and serve the example
trunk serve --release
```

We're also publicly hosting the examples at `https://examples.yew.rs/<EXAMPLE>`.
As an example, check out the TodoMVC example here: <https://examples.yew.rs/todomvc>

## List of examples

 - CT - Type of most components , "F" for function components and "S" for struct components, "SF" for a mix of both.

| Example                                            | CT | Description                                                                                                                        |
| -------------------------------------------------- | -- | ---------------------------------------------------------------------------------------------------------------------------------- |
| [agents](agents)                                   | S  | Cross-component communication using [Agents](https://yew.rs/docs/concepts/agents)                                                  |
| [async_reqwest](async_reqwest)                     | S  | Web requests asynchronously using reqwest and rendering fetched html (https://github.com/seanmonstar/reqwest)                     |
| [boids](boids)                                     | S  | Yew port of [Boids](https://en.wikipedia.org/wiki/Boids)                                                                           |
| [contexts](contexts)                               | F  | A technical demonstration of Context API.                                                                                          |
| [counter](counter)                                 | S  | Simple counter which can be incremented and decremented                                                                            |
| [dyn_create_destroy_apps](dyn_create_destroy_apps) | S  | Uses the function `start_app_in_element` and the `AppHandle` struct to dynamically create and delete Yew apps                      |
| [file_upload](file_upload)                         | S  | Uses the `gloo::file` to read the content of user uploaded files                                                                   |
| [function_todomvc](function_todomvc)               | F  | Implementation of [TodoMVC](http://todomvc.com/) using function components and hooks.                                              |
| [futures](futures)                                 | S  | Demonstrates how you can use futures and async code with Yew. Features a Markdown renderer.                                        |
| [game_of_life](game_of_life)                       | S  | Implementation of [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)                                   |
| [inner_html](inner_html)                           | S  | Embeds an external document as raw HTML by manually managing the element                                                           |
| [js_callback](js_callback)                         | S  | Interacts with JavaScript code                                                                                                     |
| [keyed_list](keyed_list)                           | S  | Demonstrates how to use keys to improve the performance of lists                                                                   |
| [mount_point](mount_point)                         | S  | Shows how to mount the root component to a custom element                                                                          |
| [nested_list](nested_list)                         | S  | Renders a styled list which tracks hover events                                                                                    |
| [node_refs](node_refs)                             | S  | Uses a [`NodeRef`](https://yew.rs/docs/concepts/components/refs) to focus the input element under the cursor                       |
| [password_strength](password_strength)             | SF | A password strength estimator implemented in Yew                                                                                   |
| [portals](portals)                                 | S  | Renders elements into out-of-tree nodes with the help of portals                                                                   |
| [router](router)                                   | S  | The best yew blog built with `yew-router`                                                                                          |
| [simple_ssr](simple_ssr)                           | F  | Demonstrates server-side rendering                                                                                                 |
| [store](store)                                     | S  | Showcases the `yewtil::store` API                                                                                                  |
| [suspense](suspense)                               | F  | This is an example that demonstrates <Suspense /> support                                                                          |
| [function_memory_game](function_memory_game)       | F  | Implementation of [Memory Game](https://github.com/bradlygreen/Memory-Game)                                            |
| [timer](timer)                                     | S  | Demonstrates the use of the interval and timeout services                                                                          |
| [todomvc](todomvc)                                 | S  | Implementation of [TodoMVC](http://todomvc.com/)                                                                                   |
| [two_apps](two_apps)                               | S  | Runs two separate Yew apps which can communicate with each other                                                                   |
| [web_worker_fib](web_worker_fib)                   | S  | Calculate fibonacci value of a number in a web worker thread                                                                       |
| [webgl](webgl)                                     | S  | Controls a [WebGL canvas](https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Getting_started_with_WebGL) from Yew |

## Next steps

Have a look at Yew's [starter templates](https://yew.rs/docs/getting-started/starter-templates) when starting a project using Yew – they can significantly simplify things.

## Help out

If one of the examples catches your interest, look for the "improvements" section in its `README` file.
Most examples list a few ideas for how to improve them.
Consider starting with those but don't hesitate to improve an example in other ways either.

One problem that currently plagues most examples is the lack of styling.
Please help us make the examples look as flashy as possible!
