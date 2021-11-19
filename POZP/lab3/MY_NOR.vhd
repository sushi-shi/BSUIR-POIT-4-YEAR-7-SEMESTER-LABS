----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    22:58:17 11/19/2021 
-- Design Name: 
-- Module Name:    MY_NOR - Behavioral 
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

entity MY_NOR is
    Port ( A : in  STD_LOGIC;
           B : in  STD_LOGIC;
           Q : out  STD_LOGIC);
end MY_NOR;

architecture Behavioral of MY_NOR is

begin
Q<= A nor B;

end Behavioral;

