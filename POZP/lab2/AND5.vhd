----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    15:58:31 11/16/2021 
-- Design Name: 
-- Module Name:    AND5 - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx primitives in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity AND_5 is
    Port ( A : in  STD_LOGIC_VECTOR (4 downto 0);
           S : out  STD_LOGIC);
end AND_5;

architecture Behavioral of AND_5 is

function reductive_and (a_vector : std_logic_vector) return std_logic is
  variable r : std_logic := '1';
begin
  for i in a_vector'range loop
    r := r and a_vector(i);
  end loop;
  return r;
end function;


begin
	S <= reductive_and(A);

end Behavioral;

