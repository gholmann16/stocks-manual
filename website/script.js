async function getPrice(id) {
    try {
        const url = '/price/' + id; 
        const response = await fetch(url);
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        
        return await response.text();
    } catch (error) {
        console.error(error.message);
    }
}

async function createStocks() {
    try {
        const response = await fetch("/stocks");
        if (!response.ok) {
            throw new Error(`Response status: ${response.status}`);
        }
        const stocks = await response.json();
        // id, name, symbol

        let parent = document.getElementById("stocks");
        let template = document.getElementById("stock");
        for (const stock of stocks) {
            let clone = template.content.cloneNode(true);

            let price = clone.querySelector(".price");
            price.textContent = await getPrice(stock.id);

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

const ctx = document.getElementById('stockGraph').getContext('2d');

const xValues = [100,200,300,400,500,600,700,800,900,1000];

new Chart(ctx, {
    type: "line",
    data: {
        labels: xValues,
        datasets: [{
                label : "Amelio",
            data: [860,1140,1060,1060,1070,1110,1330,2210,7830,2478],
            borderColor: "#FFA500",
            backgroundColor: "#FFA500",
            fill: false
        },{
                label : "Ayaan",
            data: [1600,1700,1700,1900,2000,2700,4000,5000,6000,7000],
            borderColor: "green",
            backgroundColor: "green",
            fill: false
        },{
                label : "Bibek",
            data: [1200, 1400, 1500, 1600, 1800, 1900, 2000, 2200, 2400, 2600],
            borderColor: "yellow",
            backgroundColor: "yellow",
            fill: false
        },{
                label : "Gabe",
            data: [450, 600, 650, 800, 850, 1200, 1400, 1600, 1800, 2000],
            borderColor: "#00008B",
            backgroundColor: "#00008B",
            fill: false
        },{
                label : "Jai",
            data: [300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200],
            borderColor: "#013220",
            backgroundColor: "#013220",
            fill: false
        },{
                label : "Josh",
            data: [1000, 1500, 1800, 2100, 2300, 2500, 2700, 2900, 3200, 3500],
            borderColor: "#A020F0",
            backgroundColor: "#A020F0",
            fill: false
        },{
                label : "London",
            data: [500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400],
            borderColor: "#FF69B4",
            backgroundColor: "#FF69B4",
            fill: false
        },{
                label : "Naumaan",
            data: [900, 1200, 1500, 1600, 1800, 2000, 2200, 2400, 2700, 3000],
            borderColor: "#DC143C",
            backgroundColor: "#DC143C",
            fill: false
        },
        {
                label : "Steven",
            data: [300,700,2000,5000,6000,4000,2000,1000,200,100],
            borderColor: "blue",
            backgroundColor: "blue",
            fill: false
            
         
        }]
    },
    options: {
        responsive: true,
        plugins: {
                legend: {
                    display: true,
                    labels : {
                        color: 'black',
                        font: {
                                size:16
                        },
                        boxWidth : 20,
                        boxHeight : 20,
                        usePointStyle : true,
                        fill: true,
                        padding : 15
                    }
                
         }
        }
    }
});