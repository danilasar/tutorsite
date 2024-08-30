let settings={
    tags:"h1, h2",
    pagenavigatorid:"table_of_contents",
    generateid:true,
    generatename:"header"
}

document.addEventListener('DOMContentLoaded', startup());



function startup() {
    console.info(settings.pagenavigatorid);
    let pagenavigator = document.getElementById(settings.pagenavigatorid);
    //Проверяем наличие контейнера для оглавления
    if (pagenavigator === null) {
        console.info('No container PageNavigatorJS');
        return;
    }
    let headersTags = document.querySelectorAll(settings.tags);

    //если нет заголовков, то выходим
    if(headersTags.length===0){
        console.info('No headers');
        pagenavigator.style.display = 'hidden';
        return;
    }

    //Если необходимо генерировать ид
    if(settings.generateid){
        headersTags.forEach(function (item, i){
            if(item.id===""){
                let id = `${settings.generatename}${i}`;
                item.id=id;
                console.log(id);
            }
        });
    }


    //необходима для определения вложенности списка
    let lastNumberTag;
    let content = '<ol>';

    headersTags.forEach(function (item, i, headersTags) {
        if (item.id !== "") {
            let hNumber = parseInt(item.tagName.slice(1));
            if (i == 1) {
                content += appendItem(item.id, item.innerHTML);
            }
            else {
                //Если заголовок равен прошлому заголовку, то создаём элемент списка
                if (lastNumberTag === hNumber) {
                    content += appendItem(item.id, item.innerHTML);
                }
                //Если номер тега больше прошлого, то создаём вложеный список
                else if (hNumber > lastNumberTag) {
                    let nesting = hNumber-lastNumberTag;
                    for(let r=0;r<nesting; r++){
                        content += '<ol>';
                    }
                    content += appendItem(item.id, item.innerHTML);

                }
                else if (hNumber < lastNumberTag) {
                    for (let r = 0; r < lastNumberTag - hNumber; r++) {
                        content += '</ol>';
                    }
                    content += appendItem(item.id, item.innerHTML);
                }
            }
            lastNumberTag = hNumber;
        }
        else {
            console.log('Header no find id' + item.tagName + item.innerText);
        }
    });
    //Append root list on page
    content += '</ol>';
    pagenavigator.innerHTML += content;
    console.log(content);
}


function appendItem(id, title){
    return  `<li><a href="#${id}">${title}</a></li>`;
}