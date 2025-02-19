function createOrders() {
    let orders = new Array();
    orders[0] = {id: 3, creator: "Jai", stock: "London", date: "09/16/2025", cents: 9998, kind: "Buy"}
    orders[1] = {id: 4, creator: "Bibi", stock: "Ayaan", date: "09/16/2025", cents: 3254, kind: "Sell"}

    let parent = document.getElementById("orders");
    let template = document.getElementById("order");
    for (const order of orders) {
        let clone = template.content.cloneNode(true);
        let price = clone.getElementById("price");
        price.textContent = order.stock;
        parent.appendChild(clone);
    }
}

createOrders();