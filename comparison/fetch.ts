const search_term = Deno.args.join(" ");

const response = await fetch(`https://tureng.com/en/turkish-english/${search_term}`, {
    "headers": {
        "User-Agent": "MyAgent",
    },
});

const text = await response.text();
console.log(text);
export { };

