function createOrders() {
    let orders = new Array();
    orders[0] = {id: 3, creator: "Jai", stock: "London", date: "09/16/2025", cents: 9998, kind: "Buy"}
    orders[1] = {id: 4, creator: "Bibi", stock: "Ayaan", date: "09/16/2025", cents: 3254, kind: "Sell"}
    orders[2] = {id: 5, creator: "Josh", stock: "Gabe", date: "09/18/2025", cents: 1000000, kind: "Sell"}

    let parent = document.getElementById("orders");
    let template = document.getElementById("order");
    for (const order of orders) {
        let clone = template.content.cloneNode(true);
        let price = clone.getElementById("price");
        price.textContent = order.cents;
        let stock = clone.getElementById("stock");
        stock.textContent = order.stock;
        parent.appendChild(clone);
    }
}

createOrders();

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