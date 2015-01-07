with Ada.Text_IO, Ada.Integer_Text_IO, QuickSort;
use Ada.Text_IO, Ada.Integer_Text_IO, QuickSort;

procedure Main is

   Index         : Integer := 1;
   Number        : Integer;
   Numbers       : Arr;
   
   task Printer is 
       entry Print; 
	   entry PrintValue(Value : Integer);
   end Printer;
   
   task Adder is 
       entry Add;
   end Adder;
   
   task Sorter is
      entry Sort; 
   end Sorter;

   
   task body Printer is
	    Sum : Integer;
   begin 
   
   
       accept Print;
       Put("Unsorted");
       New_Line;
	   for i in Integer range 1 .. 30 loop
		   Put(Numbers(i));
           New_Line;
	   end loop;
	  
       Sorter.Sort;
       accept Print;
       Put("Sorted");
       New_Line;
	   for i in Integer range 1 .. 30 loop
		   Put(Numbers(i));
           New_Line;
	   end loop;
	   accept PrintValue(Value : Integer) do
	   		Sum := Value;
            Put("Sum");
            New_Line;
            Put(Sum);
       end PrintValue;
   end Printer;
   
   task body Adder is
		Sum : Integer := 0;
   begin 
	   accept Add;
       for i in Integer range 1 .. 30 loop
	       Sum := Sum + Numbers(i);
       end loop;
       Printer.PrintValue(Sum);  
   end Adder;
   
   task body Sorter is      
   begin
       accept Sort;
       QuickSort.quickSort(numbers, 1, 30);
       Printer.Print;
       Adder.Add;
   end Sorter;
   	
begin

   while Index <= 30 loop
         Get(Number);
         Numbers(Index) := Number;
         Index := Index + 1;
   end loop;
  
   Printer.Print;
end Main;