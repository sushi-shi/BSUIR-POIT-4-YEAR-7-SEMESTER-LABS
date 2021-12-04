----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 05:57:53 PM
-- Design Name: 
-- Module Name: task2_reg_n_beh - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
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
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity task2_reg_n_beh is
    generic(bitness: integer := 8);
    Port (
        INIT: in STD_LOGIC;
        EN: in STD_LOGIC;
        OE: in STD_LOGIC;
        CLK: in STD_LOGIC;
        D: in STD_LOGIC_VECTOR(bitness - 1 downto 0);
        Q: out STD_LOGIC_VECTOR(bitness - 1 downto 0)
    );
end task2_reg_n_beh;

architecture Behavioral of task2_reg_n_beh is
    signal q_t : std_logic_vector(bitness - 1 downto 0);
    constant q_z : std_logic_vector(bitness - 1 downto 0) := (others => 'Z');
    constant q_zero : std_logic_vector((bitness - 1) downto 0) := (others => '0');
begin
     main : process(INIT, EN, CLK, D)
     begin
        if INIT = '1' then
            q_t <= (others => '0');
        elsif (EN = '1' and rising_edge(CLK)) then
            q_t <= D;
        end if;
    end process;
    Q <= q_t when OE = '1' else q_z;
end Behavioral;
