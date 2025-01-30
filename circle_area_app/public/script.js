function calculateArea() {
    const radius = document.getElementById('radius').value;
    fetch(`/area?radius=${radius}`)
        .then(response => response.json())
        .then(data => {
            if (data.error) {
                document.getElementById('result').innerText = data.error;
            } else {
                document.getElementById('result').innerText = `Area: ${data.area.toFixed(2)}`;
            }
        })
        .catch(error => console.error('Error:', error));
}