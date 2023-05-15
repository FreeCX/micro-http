function process() {
  fetch("/api/", {
    method: "POST",
    body: JSON.stringify({
      min: document.querySelector("input[id=min]").value,
      max: document.querySelector("input[id=max]").value
    }),
    headers: {
      "Content-Type": "application/json"
    }
  })
  .then((response) => response.json())
  .then((json) => {
    document.querySelector("span[class=current]").textContent = json.result;
  });
}
