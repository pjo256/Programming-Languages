package QuickSort is 
    type Arr is array(1..30) of Integer;
    procedure quickSort(numbers : in out Arr; low, high : in Integer);                   
end QuickSort;