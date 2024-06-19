function showOrNot(textIn, index){
    this.text = textIn;
    this.index = index;
    this.boolBack;

    if(this.text[this.index] == 0){
        this.boolBack = false;
    }else{
        this.boolBack = true;
    }

    return this.boolBack;

}