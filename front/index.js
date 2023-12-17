const URL = "http://127.0.0.1:8080";
const ending = document.getElementById('ending');
const field = document.getElementById('field');
const crossSVG = "./images/cross.png";
const circleSVG = "./images/circle.png";

var currentPlayer = true;
var gameOver = false;

const send_pos = (UserData) => {
    fetch(`${URL}/send`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(UserData),
    })
      .then(response => {
          if (response.ok) return response.json();
          else {
              return response.text().then(errorText => {
                  throw new Error(errorText);
              });
          }
      })
      .then(responseData => handleResponse(responseData))
      .catch(error => {
          alert(`Error: ${error.message}`);
          console.warn(error);
      });
};

const handleResponse = (responseData) => {
    console.log(responseData);

    if (responseData.status === 'over') {
        gameOver = true;
        
        const endingSpan = document.createElement("span");
        endingSpan.id = "endingSpan";
        endingSpan.className = "endingSpanClass"
        endingSpan.style.backgroundColor = responseData.winner === 'X' ? 
          "#bf2a2a" 
        : "#322abf";
        endingSpan.innerText = responseData.winner === 'X' ? 
          "Winner: X" 
        : "Winner: O"
        
        ending.appendChild(endingSpan); 
    }
};

const init_cells = () => {
    for (let row = 0; row < 3; row++) {
        for (let col = 0; col < 3; col++) {
            const cell = document.createElement('div');
            const button = document.createElement('button');

            cell.id = `cell.${row}.${col}`;
            button.id = `button.${row}.${col}`;
            cell.classList.add('cell');
            button.classList.add('transparent-button');
            button.addEventListener('click', () => handleFieldClick(row, col));

            cell.appendChild(button);
            field.appendChild(cell);
        }
    }
};

const handleFieldClick = (row, col) => {
    const buttonId = `button.${row}.${col}`;
    const cellId = `cell.${row}.${col}`;

    const button = document.getElementById(`${buttonId}`)
    const cell = document.getElementById(`${cellId}`);

    if (!button.classList.contains('clicked')) {
        if (gameOver) {
            alert("The game is over. Please clear the board to continue the game.");
            return;
        }

        send_pos({
            row: row,
            col: col,
            player: currentPlayer ? 'X' : 'O'
        });
        
        button.classList.add('clicked');
            
        const image = document.createElement('img');
        image.className = 'images';
        image.id = `image.${row}.${col}`;

        if (currentPlayer) image.src = crossSVG;
        else image.src = circleSVG;

        currentPlayer = !currentPlayer;

        if (image.src.includes('cross')) {
            image.classList.add('cross');
        } else if (image.src.includes('circle')) {
            image.classList.add('circle');
        }

        button.appendChild(image);

        const isCross = image.src.includes('cross.png');
        const isCircle = image.src.includes('circle.png');

        if (isCross) cell.classList.add('has-cross');
        else if (isCircle) cell.classList.add('has-circle');
    }
    //console.log(`Clicked on cell, cell id: ${cellId}, button id: ${buttonId}`);
};

const clearBoard = () => {
    gameOver = false;
    document.querySelectorAll('.has-cross .images.cross').forEach(element => {
        element.remove();
    });
    document.querySelectorAll('.has-circle .images.circle').forEach(element => {
        element.remove();
    });
    document.querySelectorAll('.container .endingContainer .endingSpanClass').forEach(element => {
        element.remove();
    });
    const clickedButtons = document.querySelectorAll('.transparent-button.clicked');
    clickedButtons.forEach(button => {
        button.classList.remove('clicked');
    });
    
    fetch(`${URL}/clear`, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify("Clear called"),
    })
      .then(response => {
          if (response.ok) {
              return response.json();
          }
          throw new Error(`HTTP error! Status: ${response.status}`);
      })
      .then(responseData => console.log(responseData))
      .catch(error => {
        alert(error.message);
        console.error(error);
      });
};

document.addEventListener('DOMContentLoaded', () => {
    init_cells();
});
