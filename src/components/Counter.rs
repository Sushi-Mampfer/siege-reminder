use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double = move || count.get() * 2;

    view! {
       <div class="flex justify-center items-center gap-x-6">
            <button on:click=move |_| *set_count.write() += 1 class="bg-teal-500 text-white px-4 py-2 rounded-xl">
                "Increment: " {count}
            </button>
            <button on:click=move |_| *set_count.write() -= 1 class="bg-pink-500 text-white px-4 py-2 rounded-xl">
                "Decrement: " {count}
            </button>
       </div>
       <div class="mt-10">
            <p class="text-center italic dark:text-white">
                "Double: " {double}
            </p>
       </div>
    }
}
