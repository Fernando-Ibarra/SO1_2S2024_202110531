console.log("Hi! From main.js on Docker container with Image 1");

const timeOutFor = async() => {
    await setTimeout(() => {
        for (let i = 0; i < 1000; i++) { 
            console.log(i)
        }
    }, 1000);
}

// random number between 0 a 250
const randomNum1 = Math.floor(Math.random() * (250 - 0) + 0);

const main = () => {
    setInterval(async() => {
        console.log("Container is running...");
        await timeOutFor();
    // 1000 -> 0.8%
    // 500 -> 1.8%
    // 100 -> 8.0% - 9.0%
    // 250 -> 3.5%
    }, randomNum1);
}

main();