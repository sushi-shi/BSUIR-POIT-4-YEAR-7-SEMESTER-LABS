----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:07:56 11/20/2021 
-- Design Name: 
-- Module Name:    SHIFT_REG - Behavioral 
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

entity SHIFT_REG is
    generic (N : integer := 8);
    port (CLR : in std_logic;
          SE : in std_logic;
          Sin : in std_logic;
          CLK : in std_logic;
          POut : out std_logic_vector(N-1 downto 0));
end SHIFT_REG;

architecture Behavioral of SHIFT_REG is

signal ff_bus : std_logic_vector(N-1 downto 0);
constant ALLZERO : std_logic_vector(N-1 downto 0) := (others => '0');

begin

  main: process(CLR, SE, CLK)
 begin
	  if (CLR = '1') then
			ff_bus <= ALLZERO;
	  elsif (SE = '1' and rising_edge(CLK)) then
			ff_bus <= Sin & ff_bus(N-2 downto 0); 
	  end if;
 end process;

 POut <= ff_bus;

end Behavioral;


