----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    20:29:58 11/17/2020 
-- Design Name: 
-- Module Name:    FSM1 - Behavioral 
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

entity FSM1 is
    port (Clk : in std_logic;
          RST : in std_logic;
          IP : in std_logic_vector(3 downto 0);
          DataOut : out std_logic_vector(1 downto 0));
end FSM1;

architecture Behavioral of FSM1 is

type states is (S0, S1, S2, S3, S4);

signal cur_state : states;
signal next_state : states;

signal output : std_logic_vector(1 downto 0);

begin

    memory : process(Clk, RST, next_state)
    begin
        if (RST = '1') then
            cur_state <= S0;
        elsif (rising_edge(Clk)) then
            cur_state <= next_state;
        end if;
    end process;

    next_state_generator : process(cur_state, IP)
    begin
        case cur_state is
            when S0 => next_state <= S1;
            when S1 => if (IP = "1101") then
                           next_state <= S2;
                       else
                           next_state <= S1;
                       end if;
            when S2 => if (IP = "1111") then
                           next_state <= S3;
                       elsif (IP = "0001") then
                           next_state <= S4;
                       else
                           next_state <= S2;
                       end if;
            when S3 => next_state <= S3;
            when S4 => if (IP = "1011") then
                           next_state <= S0;
                       elsif (IP = "1001") then
                           next_state <= S2;
                       else
                           next_state <= S4;
                       end if;
            when others => next_state <= S0;
        end case;
    end process;

    output_generator : process(cur_state)
    begin
        case cur_state is
            when S0 => output <= "00";
            when S1 | S4 => output <= "01";
            when S2 => output <= "10";
            when S3 => output <= "11";
            when others => output <= "00";
        end case;
    end process;

    DataOut <= output;

end Behavioral;

