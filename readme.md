# Shopee Log Monitor

This app is just a show case of how logging tool can make your life easy.

## Screenshots

![first](rscreenshots/1.png?raw=true "First Part")
![second](rscreenshots/2.png?raw=true "Second Part")
![third](rscreenshots/3.png?raw=true "Detail View")

# How it works?

Shopee Bot will send log to this backend log app The frontend app which is made in react will call the endpoint and render the infomation. This was necessary as no of products and keywords were growing and tracing every item/keyword was necessary.

# Screenshots

# Regarding Usage

#### Why are you using unwrap?

Because its easy. Usually this app is well monitored by human everyday so and tokio works even if there is panic due to unwrap. But regardless this backend is made in 1 day so there is no high expectation. Actually I was amazed it performed so well with node it would have taken probably 2-3 days.

#### Why warp?

Type safety is reason. Example: Change route parameter code wont compile and we know where to change. I changed model field serveral time and if code compiles usually 95% of time it runs. Rust is such a fantastic langauge.

#### Why no yew instead of React?

It's a fair compromize. I know react and the project was already taking so much time so i had to use react. And my laptop is abit potato (8 G RAM and dual core) so it would be abit hard.

### What was the hardest thing and is not implemented yet?

In my opinion making windows daemon is hard as i have no clue which I am still working. After I get a good computer I think I can make it easily. For now we are just using gitbash.

### Whats the point of repo?

Making Rest Api endpint in rust is easy and logging should be done properly . This way we already have caught 9-10 bugs

### Thanks

Thanks to Rust Team, Hyper Team and museun.

### Contact

If there is any issue contact me directly via shirshak55[at]pm.me . Replace[at] with @. My updated email shall me found in github profile (github.com/shirshak55)
