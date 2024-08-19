console.log("Hi! From main.js on Docker container with Image 4");

const timeOutFor = async() => {
    await setTimeout(() => {
        for (let i = 0; i < 1000; i++) { 
            console.log(i)
        }
    }, 1000);
}

// random number between 1000 a 2000
const randomNum4 = Math.floor(Math.random() * (2000 - 1000) + 1000);


const main = () => {
    setInterval(async() => {
        console.log("Container is running...");
        await timeOutFor();    
    }, randomNum4);
}

main();