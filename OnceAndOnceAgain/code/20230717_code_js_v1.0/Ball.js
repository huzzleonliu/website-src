class Ball {
    constructor(x, y, r, xOffset,constrain, yShake){
        //magic number
        this.r = r;
        this.yOffset;
        this.constrain = constrain;
        this.yShake = yShake;
        //related
        this.x = x;
        this.x0 = x;
        this.y = y;
        this.y0 = y;
        this.xOffset = xOffset;

    }

    show(){
        fill(255, 128);
        stroke(255);
        ellipseMode(CENTER)
        ellipse(this.x, this.y, this.r*2);
    }

    update(){
        this.yOffset = random(-this.yShake,this.yShake);
        this.x += this.xOffset;
        this.y += this.yOffset;
        //控制在上下小于constrain的范围内
        if(this.y >= this.y0 + this.constrain){
            this.y = this.y0 + this.constrain;
        }else if(this.y <= this.y0 - this.constrain){
            this.y = this.y0 - this.constrain;
        }
        //在屏幕中循环
        if(this.x < -this.r){
            this.x = width + this.r;
        }else if( this.x > width + this.r){
            this.x = -this.r;
        }
        
        
    }

}