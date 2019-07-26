
fn main() {
    let mut arr = [6,1,2,37,1,4,5];
    print!("begin:\t");
    printarray(&arr);

    let len = arr.len();
    qsort(&mut arr,0,len-1);

    print!("\nqsort:\t");
    printarray(&arr);

    heapsort(&mut arr,0,len-1);

    print!("\nheapsort:\t");
    printarray(&arr);

    let mut tmp = [0_i32;7];
    mergesort(&mut arr,&mut tmp,0,len-1);
    
    print!("\nmergesort:\t");
    printarray(&arr);
}

fn printarray(arr: &[i32]){
    for x in arr.iter(){
	print!("{} ",x);
    }
}

fn swap(arr: &mut [i32],i:usize,j:usize){
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j]= tmp;
}

fn qsort(arr: &mut [i32],start: usize,end: usize){
    if start >= end {
	return
    }
    let mut pos = start;

    for i in (start+1)..(end+1) {
	if arr[i] > arr[start]{
	    pos = pos +1;
	    swap(arr,i,pos);
	}
    }

    swap(arr,start,pos);

    if pos > 0{
	qsort(arr,start,pos-1);
    }
    qsort(arr,pos+1,end);
}

fn minheap(arr :&mut [i32],start: usize, end: usize){
    let mut start = start;
    let mut pos = start*2+1;
    while pos <= end {
	if pos+1 <= end && arr[pos+1] > arr[pos]{
	    pos = pos+1;
	}

	if arr[start] > arr[pos]{
	    break;
	}

	swap(arr,start,pos);
	start = pos;
	pos = start*2+1;
    }
}

fn heapsort(arr: &mut [i32],start: usize, end: usize){
    for i in (0..(end+1)/2).rev(){
	minheap(arr,i,end);
    }

    for i in (start..end).rev(){
	swap(arr,0,i+1);
	minheap(arr,0,i);
    }
}

fn mergearr(arr:&mut [i32],tmp:&mut [i32],start:usize,end:usize,mid:usize){
    let mut k = start;
    let mut i = start;
    let mut j = mid+1;
    while i < mid+1 && j < end+1 {
	if arr[i] < arr[j] {
	    tmp[k] = arr[j];   
	    k = k+1;
	    j = j+1;
	}else{
	    tmp[k] = arr[i];
	    k = k+1;
	    i = i+1;
	}
    }

    while i < mid+1{ 
	tmp[k] = arr[i];
	k = k+1;
	i = i+1;
    }

    while j < end+1{
	tmp[k] = arr[j];
	k = k+1;
	j = j+1;
    }

    for i in start..end+1 {
	arr[i] = tmp[i]
    }
}

fn mergesort(arr:&mut [i32],tmp:&mut [i32],start:usize,end:usize){
    if start < end {
	let mid = start + (end-start)/2;
	mergesort(arr,tmp,start,mid);
	mergesort(arr,tmp,mid+1,end);
	mergearr(arr,tmp,start,end,mid);
    }
}
