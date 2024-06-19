function lineGen(){
    //传进球中的变量
    this.constrain = 10;
    this.ballRadius = 10;
    this.yShake = 0;
    this.y = 0 + this.constrain + this.ballRadius;
    this.x;
    this.xOffset;
    //自身的变量
    this.xShake = 3;
    this.ballNumberX = 8;
    this.ballNumberY = 19;
    this.lineWidth = width / this.ballNumberX;
    this.lineHeight = (height - 2 * this.constrain - 2 * this.ballRadius) / this.ballNumberY;
    this.ballTemp = [];




// y的最高点 0 + constrian + r
// y的最低点 height - constrain - r
// 最高点-最低点 0 + constrian + r - （ height - constrain - r）
// 最高点-最低点 0 + constrian + r - height + constrain + r = - height + 2 constrain + 2 r
// 两点之间的距离 height - 2 constrain - 2 r



for(j = 0; j < this.ballNumberY; j++){
    this.x = 0 ;
    // this.xOffset = random(-(this.xShake / 3),this.xShake);
    this.xOffset = random(0,this.xShake);
    for(i=0; i < this.ballNumberX; i++){
        // console.log(this.y);
        // console.log(this.x);
        ballTemp.push(new Ball(this.x, this.y, this.ballRadius,this.xOffset, this.constrain, this.yShake));
        this.x += this.lineWidth;
    }
        this.y += this.lineHeight;

}

//console.log(this.ballTemp.length);

    return this.ballTemp;

}