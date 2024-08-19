console.log("Hi! From main.js on Docker container with Image 3");

const timeOutFor = async() => {
    await setTimeout(() => {
        for (let i = 0; i < 1000; i++) { 
            console.log(i)
        }
    }, 1000);
}


// random number between 500 a 1000
const randomNum3 = Math.floor(Math.random() * (1000 - 500) + 500);

const main = () => {
    setInterval(async() => {
        console.log("Container is running...");
        await timeOutFor();
    }, randomNum3);
}

main();