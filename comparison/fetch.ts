let args: string[] = [];
if (typeof Deno !== 'undefined') {
    // Running in Deno
    args = Deno.args;
} else if (typeof Bun !== 'undefined') {
    // Running in Deno
    args = Bun.argv.slice(2);
} else {
    console.error('Unsupported environment');
}

const search_term = args.join(" ");
const response = await fetch(`https://tureng.com/en/turkish-english/${search_term}`, {
    "headers": {
        "User-Agent": "MyAgent",
    },
});

const text = await response.text();
console.log(text);
export { };

