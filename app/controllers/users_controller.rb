class UsersController < ApplicationController
  before_action :is_matching_login_user, only: [:edit, :update]#edit,updateｱｸｼｮﾝ実行前にﾛｸﾞｲﾝ中ﾕｰｻﾞｰじゃない場合は一覧ページへﾘﾀﾞｲﾚｸﾄする
  def show
    @user = User.find(params[:id])#特定ﾕｰｻﾞｰidのﾚｺｰﾄﾞ取得部分ﾃﾝﾌﾟﾚに渡す
    @books = @user.books
    @book = Book.new#部分ﾃﾝﾌﾟﾚに渡す
  end

  def index
    @users = User.all
    @book = Book.new
    @user = current_user#部分テンプレへ渡す
  end

  def edit
    is_matching_login_user#edit,updateｱｸｼｮﾝ実行前にﾛｸﾞｲﾝ中ﾕｰｻﾞｰじゃない場合は一覧ページへﾘﾀﾞｲﾚｸﾄする
    @user = User.find(params[:id])
  end

  def update
    is_matching_login_user#edit,updateｱｸｼｮﾝ実行前にﾛｸﾞｲﾝ中ﾕｰｻﾞｰじゃない場合は一覧ページへﾘﾀﾞｲﾚｸﾄする
    @user = User.find(params[:id])
    if @user.update(user_params)#更新時条件に当てはまったら
      flash[:notice]  = "You have updated user successfully."
      redirect_to user_path(current_user.id)
    else                        #当てはまらなければ
      render :edit
    end
  end


    private
    def user_params
      params.require(:user).permit(:name, :introduction, :profile_image)
    end

    def is_matching_login_user
      user_id = params[:id].to_i#urlの数字を取得
      unless user_id == current_user.id#数字がｶﾝﾚﾄﾕｰｻﾞｰと一致しなければ
        redirect_to user_path(current_user.id)#ｶﾚﾝﾄﾕｰｻﾞｰの詳細ページに飛ぶ
      end
    end

end
