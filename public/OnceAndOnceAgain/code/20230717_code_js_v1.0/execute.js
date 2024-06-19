function execute(){
    randomSeed(floor($fx.rand() * 100));
    var ball = [];
    var OaOA = 
    "01101111"
    + "01101110"
    + "01100011"
    + "01100101"
    + "00100000"
    + "01100001"
    + "01101110"
    + "01100100"
    + "00100000"
    + "01101111"
    + "01101110"
    + "01100011"
    + "01100101"
    + "00100000"
    + "01100001"
    + "01100111"
    + "01100001"
    + "01101001"
    + "01101110";
    ball = lineGen();

    draw = function(){
        background(0);
        for(i = 0; i< ball.length; i++){
            ball[i].update();
        }
        
        for(i = 0; i< ball.length; i++){
            if(showOrNot(OaOA, i)){
                ball[i].show();
            }
        }
    }
}