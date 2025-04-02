async function createStocks() {
    try {
        const response = await fetch("/stocks");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        const stocks = await response.json();

        let parent = document.getElementById("stocks");
        let template = document.getElementById("stock");
        for (const stock of stocks) {
            let clone = template.content.cloneNode(true);

            let price = clone.querySelector(".price");
            price.textContent = stock.price;

            let stockName = clone.querySelector(".stock");
            stockName.textContent = stock.name;
            parent.appendChild(clone);
        }
    } catch (error) {
        console.error(error.message);
    }
}

createStocks();

function createOrders() {
    let orders = new Array();
    orders[0] = {id: 6, creator: "Jai", order: "London", date: "09/16/2025", cents: 9998, kind: "Buy"}
    orders[1] = {id: 7, creator: "Bibi", order: "Ayaan", date: "09/16/2025", cents: 3254, kind: "Sell"}
    orders[2] = {id: 8, creator: "Josh", order: "Gabe", date: "09/18/2025", cents: 1000000, kind: "Sell"}

    let parent = document.getElementById("orders");
    let template = document.getElementById("order");
    for (const orderObj of orders) {
        let clone = template.content.cloneNode(true);
        let price = clone.querySelector(".price");
        price.textContent = orderObj.cents;
        let orderName = clone.querySelector(".order");
        orderName.textContent = orderObj.order;
        parent.appendChild(clone);
    }
}
createOrders(); 

const buyButtons = document.querySelectorAll(".buyButton"); 
    buyButtons.forEach(button => {
    button.addEventListener("click", function() {
        alert("Button Clicked");
     }); 
}); 

const sellButtons = document.querySelectorAll(".sellButton")
    sellButtons.forEach(button => {
    button.addEventListener("click", function() {
        alert("Sell Button Clicker");
    });
});
